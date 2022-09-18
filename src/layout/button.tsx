import { _ } from '@/i18n';
import type { JSX } from 'solid-js';

import './button.scss';

export interface ButtonProps {
    children: JSX.Element | string;
    class?: string;
    onClick: () => void;
}

export const Button = (props: ButtonProps) => {
    return (
        <button
            classList={ { 'button': true, [props.class || '']: true } }
            onClick={ props.onClick }
        >
            { props.children }
        </button>
    );
};
