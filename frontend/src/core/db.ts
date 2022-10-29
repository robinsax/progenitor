export interface DBClientConfig {
    url: string;
    namespace: string;
    database: string;
    authScope: string;
}

export interface SigninParams {
    email: string;
    pass: string;
}

export interface SignupParams {
    email: string;
    pass: string;
}

type HeldHandler = [(res: unknown) => void, (err: unknown) => void];

class DBClient {
    private config: DBClientConfig;
    private socket: WebSocket;
    private socketOpen: boolean;
    private socketWaiters: (() => void)[];
    private returnHandlers: { [id: string]: HeldHandler };
    private nextId: number;
    private token: string | null;
    private tokenChangeCallbacks: ((token: string | null) => void)[];

    constructor(config: DBClientConfig) {
        this.config = config;

        this.socket = new WebSocket(config.url);
        this.socketOpen = false;
        this.socketWaiters = [];

        this.returnHandlers = {};

        this.nextId = 1;

        this.token = null;
        this.tokenChangeCallbacks = [];

        this.handleSocketOpen = this.handleSocketOpen.bind(this);
        this.socket.addEventListener('open', this.handleSocketOpen);
        this.handleSocketMessage = this.handleSocketMessage.bind(this);
        this.socket.addEventListener('message', this.handleSocketMessage);
    }

    private handleSocketOpen() {
        this.socketOpen = true;

        for (const waiter of this.socketWaiters) waiter();
    }

    private handleSocketMessage(event: MessageEvent<any>) {
        const { id, result, error } = JSON.parse(event.data);

        const handlers = this.returnHandlers[id];
        if (!handlers) return;

        if (error) handlers[1](error);
        else handlers[0](result);
    }

    private untilSocket(): Promise<void> {
        return new Promise(resolve => {
            if (this.socketOpen) resolve();
            else this.socketWaiters.push(resolve);
        });
    }

    private async transactCall(method: string, params: unknown[]): Promise<any> {
        await this.untilSocket();

        const id = `${ this.nextId++ }`;

        return new Promise((resolve, reject) => {
            this.returnHandlers[id] = [resolve, reject];

            this.socket.send(JSON.stringify({ id, method, params }));
        });
    }

    private contextParams() {
        const { database, namespace, authScope } = this.config;

        return {
            NS: namespace,
            DB: database,
            SC: authScope
        };
    }

    async connect() {
        const { database, namespace } = this.config;

        return this.transactCall('use', [namespace, database]);
    }

    async watchToken(callback: (token: string | null) => void) {
        this.tokenChangeCallbacks.push(callback);
    }

    async signup(params: SignupParams): Promise<void> {
        const resp = await this.transactCall('signup', [
            { ...params, ...this.contextParams() }
        ]);

        this.token = resp;
        for (const callback of this.tokenChangeCallbacks) {
            callback(resp);
        }
    }
    
    async signin(params: SigninParams): Promise<void> {
        const resp = await this.transactCall('signin', [
            { ...params, ...this.contextParams() }
        ]);

        this.token = resp;
        for (const callback of this.tokenChangeCallbacks) {
            callback(resp);
        }
    }

    async signout(): Promise<void> {
        this.token = null;
        for (const callback of this.tokenChangeCallbacks) {
            callback(null);
        }
    }

    async signinWithToken(token: string): Promise<void> {
        this.token = token;
        await this.transactCall('ping', []);

        for (const callback of this.tokenChangeCallbacks) {
            callback(token);
        }
    }

    async query(sql: string): Promise<any> {
        return await this.transactCall('query', [sql]);
    }
}

export const db = new DBClient({
    url: 'ws://localhost:8000/rpc',
    database: 'testdb',
    namespace: 'testns',
    authScope: 'account'
});
