export const validateEmail = (email: string) => (
    email.indexOf('@') >= 0 && email.indexOf('.') >= 0
);

export const validatePassword = (pass: string) => (
    pass.length > 8
);
