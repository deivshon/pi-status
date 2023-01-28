import './RamStat.css'

import { formatBytes } from "../../utils"

import { useState, useEffect } from 'react'

export default function Ram({total, stat, label}) {
    const [statPerc, setStatPerc] = useState(0)

    useEffect(() => {
        if(total != 0) setStatPerc((stat / total) * 100)
    }, [stat, total])
    
    return <div className="d-flex ram-stats-container flex-column w-100">
        <div className="d-flex justify-content-between ram-text">
            <div>{label}</div>
            <div>{stat != 0 ? `${formatBytes(stat, {})} (${statPerc.toFixed(2)}%)` : "0 (0%)"}</div>
        </div>
        <div class="progress w-100">
            <div class="progress-bar ram-bar" role="progressbar" style={{width: `${statPerc}%`}}></div>
        </div>
    </div>
}