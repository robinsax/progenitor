import { useContext, createContext, Accessor } from 'solid-js';

import { User } from '@/model';

export const UserContext = createContext<Accessor<User>>(null as any);

export const useUser = () => useContext(UserContext);
