import { writable } from 'svelte/store';

type User = {
  username: string;
  id: string;
  email: string;
  token: string;
};

export const userStore = writable<User>({
  username: '',
  id: '',
  email: '',
  token: '',
});


export const clearUserStore = () => {
  userStore.set({
    username: '',
    id: '',
    email: '',
    token: '',
  });
}
