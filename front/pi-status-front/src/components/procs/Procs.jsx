import { useState, useEffect } from "react"
import { formatBytes } from "../../utils"

import './Procs.css'

export default function Procs({procs, mainCpuUsage}) {
    const [currentTotal, setTotal] = useState(1)

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
    
    return <div class="stats-container flex-column">
        <div>{procs.length} active processes</div>
        <div class="proc-container d-flex flex-row">
            <div class="pid-col proc-col d-flex flex-column">
                <div className="col-content col-label">PID</div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.pid}</div>)}
            </div>
            <div class="name-col proc-col d-flex flex-column">
                <div className="col-content col-label">Name</div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.name}</div>)}
            </div>
            <div class="threads-col proc-col d-flex flex-column">
                <div className="col-content col-label">Thds</div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.threads}</div>)}
            </div>
            <div class="memory-col proc-col d-flex flex-column">
                <div className="col-content col-label">Mem</div>
                <div></div>
                {procs.map(p =>
                    <div className="col-content">
                        {formatBytes(p.mem, {short: true, space: false, roundTreshold: 10, absoluteRoundTreshold: 1024 ** 3, roundingDigits: 1})}
                    </div>
                    )}
            </div>
            <div class="cpu-col proc-col d-flex flex-column">
                <div className="col-content col-label">CPU</div>
                <div></div>
                {procs.map(p => <div className="col-content cpu-percs">{((p.cpu_usage / currentTotal) * 100).toFixed(1).padStart(5, " ")}%</div>)}
            </div>
        </div>
    </div>
}
