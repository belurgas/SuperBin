// Типы данных
export type Tab = {
  id: string;
  name: string;
  icon: string;
};

export type TestDiskStat = {
  letter: string;
  name: string;
  total: number;
  free: number;
}

export type DiskStat = {
  name: string;
  used: number;
  total: number;
  color: string;
};

export type RecentFile = {
  name: string;
  size: string;
  type: string;
  path: string;
};

export type FileTypeItem = {
  type: string;
  size: string;
  color: string;
};

export type FolderItem = {
  folder: string;
  size: string;
};

// Пропсы для компонентов
export type StatsCardProps = {
  title: string;
  value: string;
  description: string;
  icon?: string;
  color: string;
  pulse?: boolean;
};

export type ProgressBarProps = {
  value: number;
  max: number;
  colorClass: string;
};

export type FileItemProps = {
  file: RecentFile;
};