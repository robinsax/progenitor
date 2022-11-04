import { Header } from '../common';
import { OccurrencesList } from './occurrences';

export const DashboardRoute = () => {
    return (
        <div class='dashboard'>
            <Header/>
            <OccurrencesList/>
        </div>
    );
};
