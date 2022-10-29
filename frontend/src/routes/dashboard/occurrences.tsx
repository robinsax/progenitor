import { createSignal, For, onMount, Show } from 'solid-js';

import { Occurrence } from '@/model';
import { db } from '@/core';
import { _ } from '@/i18n';
import { ErrorView, Group, Hint, TextField } from '@/layout';

import './occurrences.scss';

interface OccurrenceItemProps {
    occurrence: Occurrence;
}

const OccurrenceItem = (props: OccurrenceItemProps) => {
    return (
        <div class='occurrence'>
            <h4>{ props.occurrence.title }</h4>
            <div class='occurrence-desc'>
                { props.occurrence.description }
            </div>
        </div>
    );
};

const OccurrenceForm = () => {
    const [titleInput, setTitleInput] = createSignal<string>('');
    const [descInput, setDescInput] = createSignal<string>('');
    const [submitError, setSubmitError] = createSignal<string | null>(null);

    const handleSubmit = async () => {
        const title = titleInput();
        const desc = descInput();

        try {
            await db.query(`CREATE occurrence SET title = ${ JSON.stringify(title) }, description = ${ JSON.stringify(desc) };`);
        }
        catch (err) {
            setSubmitError(err + '');
        }
    };

    return (
        <div class='occurrence-form'>
            <h3>{ _('Create event') }</h3>
            <Group>
                <Hint>{ _('Title') }</Hint>
                <TextField
                    value={ titleInput() }
                    invalid={ false }
                    onChange={ setTitleInput }
                    onEnter={ handleSubmit }
                />
            </Group>
            <Group>
                <Hint>{ _('Description') }</Hint>
                <TextField
                    value={ descInput() }
                    invalid={ false }
                    onChange={ setDescInput }
                    onEnter={ handleSubmit }
                />
            </Group>
            <ErrorView error={ submitError() }/>
        </div>
    )
};

interface CommentFormProps {
    occurrence: Occurrence;
}

const CommentForm = (props: CommentFormProps) => {
    const [textInput, setTextInput] = createSignal<string>('');
    const [submitError, setSubmitError] = createSignal<string | null>(null);

    const handleSubmit = async () => {
        const text = textInput();

        try {
            await db.query(`CREATE comment SET occurence = ${ props.occurrence.id }, text = ${ JSON.stringify(text) };`);
        }
        catch (err) {
            setSubmitError(err + '');
        }
    };

    return (
        <div class='occurrence-form'>
            <h3>{ _('Create event') }</h3>
            <Group>
                <Hint>{ _('Comment') }</Hint>
                <TextField
                    value={ textInput() }
                    invalid={ false }
                    onChange={ setTextInput }
                    onEnter={ handleSubmit }
                />
            </Group>
            <ErrorView error={ submitError() }/>
        </div>
    )
};

export const OccurrencesList = () => {
    const [occurrences, setOccurrences] = createSignal<Occurrence[] | null>(null);

    onMount(async () => {
        const resp = await db.query('SELECT id, title, description, <-comments FROM occurrence LIMIT 10;');

        setOccurrences(resp[0].result);
    });

    return (
        <div class='occurrences-list'>
            <div>
                { (set => (
                    !set ? '...' : !set.length ? 'Nothing to show' :
                        <For each={ occurrences() }>{ item => (
                            <OccurrenceItem occurrence={ item }/>
                        ) }</For>
                ))(occurrences()) }
            </div>
            <OccurrenceForm/>
        </div>
    )
};