import { useNavigate } from '@solidjs/router';

import { _ } from '@/i18n';

import './link.scss';

export interface LinkProps {
    children: string;
    route: string;
    class?: string;
}

export const Link = (props: LinkProps) => {
    const navigate = useNavigate();

    const handleClick = (event: MouseEvent) => {
        event.preventDefault();

        navigate(props.route);
    };

    return (
        <a
            classList={ { 'link': true, [props.class || '']: true } }
            href={ props.route }
            onClick={ handleClick }
        >
            { props.children }
        </a>
    );
};

export const LeaveFlowLink = () => {
    return (
        <Link route='/' class='leave-flow'>
            { _('<') }
        </Link>
    )
};