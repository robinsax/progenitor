export type I18nFn = (key: string, ...values: (string | number)[]) => string;

export const _: I18nFn = (key: string, ...values: (string | number)[]): string => {
    for (let i = 0; i < values.length; i++) {
        key = key.split(`{${ i }}`).join(values[i] + '');
    }

    return key;
};
