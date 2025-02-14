export interface Task {
    name: string;
    position: number;
    quadrant: number;
}

export const fetchTasks = async (): Promise<Task[]> =>  {
    const response = await fetch('http://127.0.0.1:8080/tasks');
    if (!response.ok) {
        throw new Error('Failed to fetch tasks');
    }
    return response.json();
};

export const addTask = async (task: Task): Promise<Task[]> => {
    const response = await fetch('http://127.0.0.1:8080/tasks/add', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(task),
    });
    if (!response.ok) {
        throw new Error('Failed to add task');
    }
    return response.json();
};

export const deleteTask = async (position: number): Promise<Task[]> => {
    const response = await fetch(`http://127.0.0.1:8080/tasks/delete/${position}`, {
        method: 'DELETE',
    });

    if (!response.ok) {
        throw new Error('Failed to delete task');
    }
    return response.json();
};

export const completeTask = async (): Promise<Task[]> => {
    const response = await fetch('http://127.0.0.1:8080/tasks/complete', {
        method: 'POST',
    });

    if (!response.ok) {
        throw new Error('Failed to complete task');
    }
    return response.json();
};