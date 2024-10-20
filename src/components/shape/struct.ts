export interface ShapeBody {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  fill: string;
  shadowBlur?: number;
  draggable?: boolean;
  rotation?: number;
  shadowColor?: string;
  shadowOpacity?: number;
  shadowOffsetX?: number;
  shadowOffsetY?: number;
  scaleX?: number;
  scaleY?: number;
  onDragStart?: (any?: any) => void;
  onDragEnd?: (any?: any) => void;
  onDragMove?: (any?: any) => void;
  stroke?: string;
}
