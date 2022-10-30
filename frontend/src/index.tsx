import { createSignal, onMount } from 'solid-js';
import { render } from 'solid-js/web';
import { Route, Router, Routes } from '@solidjs/router';
import jwtDecode from 'jwt-decode';

import { UserContext, db, NoUserGaurd } from '@/core';
import { ErrorRoute, SignInRoute, SignUpRoute, DashboardRoute } from '@/routes';
import { User } from '@/model';

import './index.scss';

const View = () => {
    const [user, setUser] = createSignal<User>(null as any);

    onMount(() => {
        db.watchToken(async token => {
            if (token) {
                const { id } = jwtDecode<{ id: string }>(token);

                const res = await db.query(`SELECT id, email FROM user WHERE id = ${ id }`);

                setUser(res[0].result[0]);
            }
            else {
                setUser(null as any);
            }
        });
    });

    return (
        <UserContext.Provider value={ user }>
            <Routes>
                <Route path='*' component={ ErrorRoute }/>
                <Route path='/' component={ DashboardRoute }/>
                <Route path='/auth' component={ NoUserGaurd }>
                    <Route path='/sign-in' component={ SignInRoute }/>
                    <Route path='/sign-up' component={ SignUpRoute }/>
                </Route>
            </Routes>
        </UserContext.Provider>
    );
};

(async () => {
    await db.connect();

    const initToken = localPersistence.getItem('jwt');
    if (initToken) {
        db.signinWithToken(initToken);
    }

    db.watchToken(token => {
        localPersistence.setItem('jwt', token || '');
    });

    render(() => (
        <Router>
            <View />
        </Router>
    ), document.getElementById('root') as HTMLElement);
})();
