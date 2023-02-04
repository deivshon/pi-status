import { useState, useEffect, useMemo } from "react"
import { formatBytes } from "../../utils"

import './Procs.css'

const PID = 0
const NAME = 1
const THREADS = 2
const MEM = 3
const CPU = 4

const ORDERINGS = [
    (p1, p2) => p2.pid - p1.pid,
    (p1, p2) => p1.name.localeCompare(p2.name),
    (p1, p2) => p2.threads - p1.threads,
    (p1, p2) => p2.mem - p1.mem,
    (p1, p2) => p2.cpu_usage - p1.cpu_usage
]

export default function Procs({procs, mainCpuUsage}) {
    const [currentTotal, setTotal] = useState(1)
    const [ordering, setOrdering] = useState({
        ord: MEM,
        rev: false
    })

    useEffect(() => {
        setTotal(
            mainCpuUsage.user +
            mainCpuUsage.system +
            mainCpuUsage.irq +
            mainCpuUsage.softirq +
            mainCpuUsage.idle +
            mainCpuUsage.iowait
        )

    }, [mainCpuUsage])

    useMemo(() => {
        const sortProcs = () => {
            if(ordering.rev) procs = procs.sort(ORDERINGS[ordering.ord]).reverse()
            else procs = procs.sort(ORDERINGS[ordering.ord])
        }

        sortProcs()
    }, [procs, ordering])

    const sortBy = propId => {
        setOrdering({
            ord: propId,
            rev: ordering.ord == propId ? !ordering.rev : false
        })
    }

    return <div class="stats-container flex-column">
        <div>{procs.length} active processes</div>
        <div class="proc-container d-flex flex-row">
            <div class="pid-col proc-col d-flex flex-column">
                <div
                className="text-nowrap"
                onClick={() => sortBy(PID)}>
                    {ordering.ord === PID ? (ordering.rev ? "↑" : "↓") : ""}
                    <span className={`col-content${ordering.ord == PID ? " text-decoration-underline" : ""}`}>PID</span>
                </div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.pid}</div>)}
            </div>
            <div class="name-col proc-col d-flex flex-column">
                <div
                className="text-nowrap"
                onClick={() => sortBy(NAME)}>
                    <span className={`col-content${ordering.ord == NAME ? " text-decoration-underline" : ""}`}>Name</span>{ordering.ord === NAME ? (ordering.rev ? "↑" : "↓") : ""}
                </div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.name}</div>)}
            </div>
            <div class="threads-col proc-col d-flex flex-column">
                <div
                className="text-nowrap threads-label"
                onClick={() => sortBy(THREADS)}>
                    <span className={`col-content${ordering.ord == THREADS ? " text-decoration-underline" : ""}`}>Thds</span>
                    {ordering.ord === THREADS ? (ordering.rev ? "↑" : "↓") : ""}
                </div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.threads}</div>)}
            </div>
            <div class="memory-col proc-col d-flex flex-column">
                <div
                className="text-nowrap mem-label"
                onClick={() => sortBy(MEM)}>
                    <span className={`col-content${ordering.ord == MEM ? " text-decoration-underline" : ""}`}>Mem</span>
                    {ordering.ord === MEM ? (ordering.rev ? "↑" : "↓") : ""}
                </div>
                <div></div>
                {procs.map(p =>
                    <div className="col-content">
                        {formatBytes(p.mem, {short: true, space: false, roundTreshold: 10, absoluteRoundTreshold: 1024 ** 3, roundingDigits: 1})}
                    </div>
                    )}
            </div>
            <div class="cpu-col proc-col d-flex flex-column">
                <div
                className="text-nowrap cpu-label"
                onClick={() => sortBy(CPU)}>
                    <span className={`col-content${ordering.ord == CPU ? " text-decoration-underline" : ""}`}>CPU</span>
                    {ordering.ord === CPU ? (ordering.rev ? "↑" : "↓") : ""}
                </div>
                <div></div>
                {procs.map(p => <div className="col-content cpu-percs">{((p.cpu_usage / currentTotal) * 100).toFixed(1).padStart(5, " ")}%</div>)}
            </div>
        </div>
    </div>
}
