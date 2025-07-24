import { writable } from 'svelte/store';

function createToastStore() {
  const { subscribe, update } = writable([]);

  let nextId = 1;

  function addToast(toast) {
    const id = nextId++;
    const newToast = {
      id,
      type: toast.type || 'info',
      message: toast.message,
      duration: toast.duration || 4000,
      ...toast
    };

    update(toasts => [...toasts, newToast]);

    if (newToast.duration > 0) {
      setTimeout(() => {
        removeToast(id);
      }, newToast.duration);
    }

    return id;
  }

  function removeToast(id) {
    update(toasts => toasts.filter(t => t.id !== id));
  }

  function clearToasts() {
    update(() => []);
  }

  return {
    subscribe,
    add: addToast,
    remove: removeToast,
    clear: clearToasts
  };
}

export const toastStore = createToastStore();
export const addToast = toastStore.add.bind(toastStore);