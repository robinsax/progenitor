import './error-view.scss';

export interface ErrorViewProps {
    error: string | null;
}

export const ErrorView = (props: ErrorViewProps) => {
    return (
        <>
            { props.error && (
                <div class='error-view'>
                    { props.error }
                </div>
            ) }
        </>
    );
};
