// App.tsx

import React, { useState, useEffect } from 'react';
import { List, Typography } from 'antd';

interface Todo {
  id: number;
  text: string;
}

const App: React.FC = () => {
  const [todos, setTodos] = useState<Todo[]>([]);

  useEffect(() => {
    fetch('/todos')
      .then(res => res.json())
      .then(data => setTodos(data));
  }, []);

  return (
    <List
      header={<Typography.Title level={3}>Todos</Typography.Title>}
      bordered
      dataSource={todos}
      renderItem={(item: Todo) => (
        <List.Item>{item.text}</List.Item>
      )}
    />
  );
}

export default App;