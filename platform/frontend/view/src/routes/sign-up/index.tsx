import { createSignal } from 'solid-js';

import { db } from '@/core';
import { Brand, CenterPanel, ErrorView, Group, TextField, Hint, Button, Link, LeaveFlowLink } from '@/layout';
import { validateEmail, validatePassword } from '@/model';
import { _ } from '@/i18n';

import './index.scss';

export const SignUpRoute = () => {
    const [emailInput, setEmailInput] = createSignal<[string, boolean]>(['', false]);
    const [passInput, setPassInput] = createSignal<[string, boolean]>(['', false]);
    const [submitted, setSubmitted] = createSignal(false);
    const [submitError, setSubmitError] = createSignal<string | null>(null);
    
    const handleEmailChange = (email: string) => {
        setSubmitted(false);
        setSubmitError(null);
        setEmailInput([email, validateEmail(email)]);
    };

    const handlePassChange = (pass: string) => {
        setSubmitted(false);
        setSubmitError(null);
        setPassInput([pass, validatePassword(pass)]);
    };

    const handleSubmit = async () => {
        const [email, emailValid] = emailInput();
        const [pass, passValid] = passInput();

        setSubmitted(true);

        if (!emailValid || !passValid) return;

        try {
            await db.signup({ email, pass });
        }
        catch (err) {
            setSubmitError(err + '');
        }
    };

    return (
        <CenterPanel>
            <div class='sign-up-form'>
                <LeaveFlowLink/>
                <Group>
                    <h3><Brand/> { _('Create account') }</h3>
                </Group>
                <ErrorView error={ submitError() }/>
                <Group align='left'>
                    <Hint>{ _('Email') }</Hint>
                    <TextField
                        value={ emailInput()[0] }
                        invalid={ submitted() && !emailInput()[1] }
                        onChange={ handleEmailChange }
                        onEnter={ handleSubmit }
                    />
                </Group>
                <Group align='left'>
                    <Hint>{ _('Password') }</Hint>
                    <TextField
                        value={ passInput()[0] }
                        invalid={ submitted() && !passInput()[1] }
                        onChange={ handlePassChange }
                        onEnter={ handleSubmit }
                    />
                </Group>
                <Group align={ ['left', 'bottom'] } width='half'>
                    <Hint>
                        <Link route='/auth/sign-in'>
                            { _('Already have an account?') }
                        </Link>
                    </Hint>
                </Group>
                <Group align='right' width='half'>
                    <Group>
                        <Button onClick={ handleSubmit }>
                            { _('Sign up') }
                        </Button>
                    </Group>
                </Group>
            </div>
        </CenterPanel>
    );
};
