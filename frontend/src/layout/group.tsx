import type { JSX } from 'solid-js';

import './group.scss';

export type GroupAlign = (
    'left' | 'right' | 'bottom'
)

export interface GroupProps {
    children: JSX.Element;
    align?: GroupAlign | GroupAlign[];
    width?: 'half' | 'third' | 'quarter';
}

export const Group = (props: GroupProps) => {
    const aligns = (
        !props.align ? [] : (
            props.align instanceof Array ? props.align : [props.align]
        )
    );

    return (
        <div
            classList={ {
                'group': true,
                [props.width ? 'size-' + props.width : '']: true,
                ...aligns.reduce((res, key) => ({ ...res, ['align-' + key]: true }), {})
            } }
        >
            { props.children }
        </div>
    );
};
