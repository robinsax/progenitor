import './text-field.scss';

export interface TextFieldProps {
    value: string;
    invalid: boolean;
    onChange: (value: string) => void;
    onEnter: () => void;
}

export const TextField = (props: TextFieldProps) => {
    const handleKeyUp = (event: KeyboardEvent) => {
        if (event.code == 'Enter') {
            props.onEnter();
            return;
        }

        props.onChange((event.target as HTMLInputElement).value);
    };

    return (
        <div class='text-field-container'>
            <input
                classList={ {'text-field': true, 'invalid': props.invalid} }
                type='text'
                value={ props.value }
                onKeyUp={ handleKeyUp }
            />
        </div>
    )
};
