import type { JSX } from 'solid-js';

import './hint.scss';

export interface HintProps {
    children: JSX.Element | string;
}

export const Hint = (props: HintProps) => {
    return (
        <div class='hint'>
            { props.children }
        </div>
    );
};
