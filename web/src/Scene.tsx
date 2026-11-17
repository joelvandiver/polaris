import { useRef, useState } from 'react'

type Pt = { x: number; y: number }

export function Scene() {
  const svgRef = useRef<SVGSVGElement>(null)
  const [points, setPoints] = useState<Pt[]>([])
  const [view] = useState({ x: -10, y: -10, w: 20, h: 20 })

  const toPlot = (e: React.PointerEvent): Pt => {
    const svg = svgRef.current!
    const p = new DOMPoint(e.clientX, e.clientY).matrixTransform(
      svg.getScreenCTM()!.inverse(),
    )
    return { x: p.x, y: -p.y };
  }

  return (
    <svg
      ref={svgRef}
      viewBox={`${view.x} ${view.y} ${view.w} ${view.h}`}
      style={{ width: '100%', height: '80vh', background: '#fafafa' }}
      onPointerDown={(e) => setPoints((ps) => [...ps, toPlot(e)])}
    >
      <g transform="scale(1,-1)">
        <line x1={view.x} y1={0} x2={view.x + view.w} y2={0} stroke="#ccc" vectorEffect="non-scaling-stroke" />
        <line x1={0} y1={view.y} x2={0} y2={view.y + view.h} stroke="#ccc" vectorEffect="non-scaling-stroke" />

        {points.map((p , i) => {
          <circle key={i} cx={p.x} cy={p.y} r={0.15} fill="steelblue" />
        })}
      </g>
    </svg>
    )
}
