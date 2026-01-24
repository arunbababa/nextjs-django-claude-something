'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/contexts/AuthContext';
import { api, Task } from '@/lib/api';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import { Trash2, Plus, LogOut } from 'lucide-react';

export default function TasksPage() {
  const { user, loading: authLoading, logout } = useAuth();
  const router = useRouter();
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [newTaskTitle, setNewTaskTitle] = useState('');
  const [newTaskDescription, setNewTaskDescription] = useState('');
  const [editingTask, setEditingTask] = useState<number | null>(null);
  const [editTitle, setEditTitle] = useState('');
  const [editDescription, setEditDescription] = useState('');

  useEffect(() => {
    if (!authLoading && !user) {
      router.push('/login');
    }
  }, [user, authLoading, router]);

  useEffect(() => {
    if (user) {
      loadTasks();
    }
  }, [user]);

  const loadTasks = async () => {
    try {
      const data = await api.getTasks();
      setTasks(data);
    } catch (error) {
      console.error('Failed to load tasks:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateTask = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTaskTitle.trim()) return;

    try {
      const task = await api.createTask(newTaskTitle, newTaskDescription);
      setTasks([task, ...tasks]);
      setNewTaskTitle('');
      setNewTaskDescription('');
    } catch (error) {
      console.error('Failed to create task:', error);
    }
  };

  const handleToggleComplete = async (task: Task) => {
    try {
      const updated = await api.updateTask(task.id, { completed: !task.completed });
      setTasks(tasks.map(t => t.id === task.id ? updated : t));
    } catch (error) {
      console.error('Failed to update task:', error);
    }
  };

  const handleDeleteTask = async (id: number) => {
    try {
      await api.deleteTask(id);
      setTasks(tasks.filter(t => t.id !== id));
    } catch (error) {
      console.error('Failed to delete task:', error);
    }
  };

  const handleStartEdit = (task: Task) => {
    setEditingTask(task.id);
    setEditTitle(task.title);
    setEditDescription(task.description);
  };

  const handleSaveEdit = async (id: number) => {
    try {
      const updated = await api.updateTask(id, {
        title: editTitle,
        description: editDescription,
      });
      setTasks(tasks.map(t => t.id === id ? updated : t));
      setEditingTask(null);
    } catch (error) {
      console.error('Failed to update task:', error);
    }
  };

  const handleCancelEdit = () => {
    setEditingTask(null);
    setEditTitle('');
    setEditDescription('');
  };

  const handleLogout = async () => {
    try {
      await logout();
      router.push('/login');
    } catch (error) {
      console.error('Failed to logout:', error);
    }
  };

  if (authLoading || loading) {
    return (
      <div className="flex min-h-screen items-center justify-center">
        <div className="text-lg">読み込み中...</div>
      </div>
    );
  }

  if (!user) {
    return null;
  }

  return (
    <div className="min-h-screen bg-slate-50 py-8">
      <div className="max-w-4xl mx-auto px-4">
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-3xl font-bold text-slate-900">タスク管理</h1>
            <p className="text-slate-600 mt-1">ようこそ、{user.username}さん</p>
          </div>
          <Button variant="outline" onClick={handleLogout}>
            <LogOut className="w-4 h-4 mr-2" />
            ログアウト
          </Button>
        </div>

        <Card className="mb-8">
          <CardHeader>
            <CardTitle>新しいタスクを追加</CardTitle>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleCreateTask} className="space-y-4">
              <div>
                <Input
                  placeholder="タスクのタイトル"
                  value={newTaskTitle}
                  onChange={(e) => setNewTaskTitle(e.target.value)}
                  required
                />
              </div>
              <div>
                <Input
                  placeholder="タスクの詳細（任意）"
                  value={newTaskDescription}
                  onChange={(e) => setNewTaskDescription(e.target.value)}
                />
              </div>
              <Button type="submit" className="w-full">
                <Plus className="w-4 h-4 mr-2" />
                タスクを追加
              </Button>
            </form>
          </CardContent>
        </Card>

        <div className="space-y-4">
          {tasks.length === 0 ? (
            <Card>
              <CardContent className="pt-6">
                <p className="text-center text-slate-500">
                  タスクがありません。新しいタスクを追加してください。
                </p>
              </CardContent>
            </Card>
          ) : (
            tasks.map((task) => (
              <Card key={task.id}>
                <CardContent className="pt-6">
                  {editingTask === task.id ? (
                    <div className="space-y-4">
                      <Input
                        value={editTitle}
                        onChange={(e) => setEditTitle(e.target.value)}
                      />
                      <Input
                        value={editDescription}
                        onChange={(e) => setEditDescription(e.target.value)}
                        placeholder="詳細"
                      />
                      <div className="flex gap-2">
                        <Button onClick={() => handleSaveEdit(task.id)} size="sm">
                          保存
                        </Button>
                        <Button onClick={handleCancelEdit} variant="outline" size="sm">
                          キャンセル
                        </Button>
                      </div>
                    </div>
                  ) : (
                    <div className="flex items-start gap-4">
                      <Checkbox
                        checked={task.completed}
                        onChange={() => handleToggleComplete(task)}
                        className="mt-1"
                      />
                      <div className="flex-1 cursor-pointer" onClick={() => handleStartEdit(task)}>
                        <h3 className={`font-medium ${task.completed ? 'line-through text-slate-500' : 'text-slate-900'}`}>
                          {task.title}
                        </h3>
                        {task.description && (
                          <p className="text-sm text-slate-600 mt-1">{task.description}</p>
                        )}
                        <p className="text-xs text-slate-400 mt-2">
                          作成日: {new Date(task.created_at).toLocaleString('ja-JP')}
                        </p>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => handleDeleteTask(task.id)}
                      >
                        <Trash2 className="w-4 h-4 text-red-500" />
                      </Button>
                    </div>
                  )}
                </CardContent>
              </Card>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
