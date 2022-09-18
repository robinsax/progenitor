import { useUser } from '@/core';
import { _ } from '@/i18n';
import { Brand, Group, Link } from '@/layout';

import './header.scss';

export const Header = () => {
    const user = useUser();

    return (
        <div class='header'>
            <Group width='third' align='left'>
                <Brand/>
            </Group>
            <Group width='third'>
                
            </Group>
            <Group width='third' align='right'>
                { user() ?
                    <div class='user-info'>
                        { user().email }
                    </div>
                    :
                    <Link route='/auth/sign-in' class='button'>
                        { _('Sign in') }
                    </Link>
                }
            </Group>
        </div>
    );
};
