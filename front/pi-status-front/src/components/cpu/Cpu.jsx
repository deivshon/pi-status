import './Cpu.css'

import CpuBar from './CpuBar'

export default function Cpu({temp, cpuUsage}) {
    return <div class="stats-container cpu-stats-container">
        <div className="temp">
            {temp}°C
        </div>
        <div className="bars-container w-100 align-items-center">
            {
                cpuUsage.length != 0 ?
                <div class="core-container">
                        <div>All</div>
                        <CpuBar coreUsage={cpuUsage[0]} />
                </div> 
                :
                ""
            }
            <div></div>
            {cpuUsage.slice(1).map((coreUsage, index) => <div class="core-container">
                    <div>Core {index + 1}</div>
                    <CpuBar coreUsage={coreUsage} />
                </div> 
            )}
        </div>
    </div>
}
