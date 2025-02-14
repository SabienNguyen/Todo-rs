import { useEffect, useState } from 'react';
import { Task, fetchTasks, addTask, deleteTask, completeTask } from './services/api';

function App() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    fetchTasks().then(setTasks).catch(console.error);
  }, []);

  const handleAddTask = async () => {
    const newTask = { name: 'New Task', position: 0, quadrant: 1 };
    const updatedTasks = await addTask(newTask);
    setTasks(updatedTasks);
  };

  const handleDeleteTask = async (position: number) => {
    const updatedTasks = await deleteTask(position);
    setTasks(updatedTasks);
  };

  const handleCompleteTask = async () => {
    const updatedTasks = await completeTask();
    setTasks(updatedTasks);
  };

  return (
    <div>
      <h1>Task Manager</h1>
      <button onClick={handleAddTask}>Add Task</button>
      <button onClick={handleCompleteTask}>Complete Task</button>
      <ul>
        {tasks.map((task) => (
          <li key={task.position}>
            {task.name}
            <button onClick={() => handleDeleteTask(task.position)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;