import { formatBytes } from "../../utils"

import './Procs.css'

export default function Procs({procs}) {
    return <div class="stats-container flex-column">
        <div>{procs.length} active processes</div>
        <div class="proc-container d-flex flex-row p-2">
            <div class="pid-col proc-col d-flex flex-column">
                <div className="col-content">PID</div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.pid}</div>)}
            </div>
            <div class="name-col proc-col d-flex flex-column">
                <div className="col-content">Name</div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.name}</div>)}
            </div>
            <div class="memory-col proc-col d-flex flex-column">
                <div className="col-content">Mem</div>
                <div></div>
                {procs.map(p => <div className="col-content">{formatBytes(p.mem, false, true, 10)}</div>)}
            </div>
            <div class="threads-col proc-col d-flex flex-column">
                <div className="col-content">Threads</div>
                <div></div>
                {procs.map(p => <div className="col-content">{p.threads}</div>)}
            </div>
        </div>
    </div>
}
