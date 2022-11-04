import type { JSX } from 'solid-js';

import './center-panel.scss';

export interface CenterPanelProps {
    children: JSX.Element;
}

export const CenterPanel = (props: CenterPanelProps) => {
    return (
        <div class='center-panel-container'>
            <div class='center-panel'>
                { props.children }
            </div>
        </div>
    );
};
