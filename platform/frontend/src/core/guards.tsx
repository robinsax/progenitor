import { Outlet, useNavigate } from '@solidjs/router';
import { createEffect, Show } from 'solid-js';

import { useUser } from './contexts';

export const UserRouteGaurd = () => {
    const navigate = useNavigate();
    const user = useUser();

    createEffect(() => {
        if (!user()) {
            navigate('/auth/sign-in', { replace: true });
        }
    });

    return (
        <Show when={ !!user() }>
            <Outlet/>
        </Show>
    );
};

export const NoUserGaurd = () => {
    const navigate = useNavigate();
    const user = useUser();
    
    createEffect(() => {
        if (user()) {
            navigate('/', { replace: true });
        }
    });
    
    return (
        <Show when={ !user() }>
            <Outlet/>
        </Show>
    );
};
