import { useState, useEffect, useMemo } from "react";

import { formatBytes } from "../../utils";
import { CoreData } from "../cpu/models";
import "./Procs.css";

enum ProcessProperty {
    PID = 0,
    NAME = 1,
    THREADS = 2,
    MEM = 3,
    CPU = 4,
}

type ProcessOrderingFunction = (p1: ProcessData, p2: ProcessData) => number;
const orderingFromProperty = (
    processProperty: ProcessProperty
): ProcessOrderingFunction => {
    switch (processProperty) {
        case ProcessProperty.PID:
            return (p1, p2) => p2.pid - p1.pid;
        case ProcessProperty.NAME:
            return (p1, p2) => p1.name.localeCompare(p2.name);
        case ProcessProperty.THREADS:
            return (p1, p2) => p2.threads - p1.threads;
        case ProcessProperty.MEM:
            return (p1, p2) => p2.mem - p1.mem;
        case ProcessProperty.CPU:
            return (p1, p2) => p2.cpu_usage - p1.cpu_usage;
        default:
            console.log(`Unknown ordering value passed: ${processProperty}`);
            return (_p1, _p2) => 0;
    }
};

type ProcProps = {
    procs: ProcessData[];
    mainCpuUsage: CoreData;
};

export default function Procs(props: ProcProps) {
    const [currentTotal, setTotal] = useState(1);
    const [ordering, setOrdering] = useState({
        ord: ProcessProperty.MEM,
        rev: false,
    });

    useEffect(() => {
        setTotal(
            props.mainCpuUsage.user +
                props.mainCpuUsage.nice +
                props.mainCpuUsage.system +
                props.mainCpuUsage.idle +
                props.mainCpuUsage.iowait +
                props.mainCpuUsage.irq +
                props.mainCpuUsage.softirq +
                props.mainCpuUsage.steal +
                props.mainCpuUsage.guest +
                props.mainCpuUsage.guest_nice
        );
    }, [props.mainCpuUsage]);

    useMemo(() => {
        const sortProcs = () => {
            if (ordering.rev)
                props.procs = props.procs
                    .sort(orderingFromProperty(ordering.ord))
                    .reverse();
            else
                props.procs = props.procs.sort(
                    orderingFromProperty(ordering.ord)
                );
        };

        sortProcs();
    }, [props.procs, ordering]);

    const sortBy = (propId: ProcessProperty) => {
        setOrdering({
            ord: propId,
            rev: ordering.ord == propId ? !ordering.rev : false,
        });
    };

    return (
        <div className="stats-container flex-column">
            <div>{props.procs.length} active processes</div>
            <div className="proc-container d-flex flex-row">
                <div className="pid-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap"
                        onClick={() => sortBy(ProcessProperty.PID)}
                    >
                        {ordering.ord === ProcessProperty.PID
                            ? ordering.rev
                                ? "↑"
                                : "↓"
                            : ""}
                        <span
                            className={`col-content${
                                ordering.ord == ProcessProperty.PID
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            PID
                        </span>
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">{p.pid}</div>
                    ))}
                </div>
                <div className="name-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap"
                        onClick={() => sortBy(ProcessProperty.NAME)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord == ProcessProperty.NAME
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            Name
                        </span>
                        {ordering.ord === ProcessProperty.NAME
                            ? ordering.rev
                                ? "↑"
                                : "↓"
                            : ""}
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">{p.name}</div>
                    ))}
                </div>
                <div className="threads-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap threads-label"
                        onClick={() => sortBy(ProcessProperty.THREADS)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord == ProcessProperty.THREADS
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            Thds
                        </span>
                        {ordering.ord === ProcessProperty.THREADS
                            ? ordering.rev
                                ? "↑"
                                : "↓"
                            : ""}
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">{p.threads}</div>
                    ))}
                </div>
                <div className="memory-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap mem-label"
                        onClick={() => sortBy(ProcessProperty.MEM)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord == ProcessProperty.MEM
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            Mem
                        </span>
                        {ordering.ord === ProcessProperty.MEM
                            ? ordering.rev
                                ? "↑"
                                : "↓"
                            : ""}
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content">
                            {formatBytes(p.mem, {
                                short: true,
                                space: false,
                                roundTreshold: 10,
                                absoluteRoundTreshold: 1024 ** 3,
                                roundingDigits: 1,
                            })}
                        </div>
                    ))}
                </div>
                <div className="cpu-col proc-col d-flex flex-column">
                    <div
                        className="text-nowrap cpu-label"
                        onClick={() => sortBy(ProcessProperty.CPU)}
                    >
                        <span
                            className={`col-content${
                                ordering.ord == ProcessProperty.CPU
                                    ? " text-decoration-underline"
                                    : ""
                            }`}
                        >
                            CPU
                        </span>
                        {ordering.ord === ProcessProperty.CPU
                            ? ordering.rev
                                ? "↑"
                                : "↓"
                            : ""}
                    </div>
                    <div></div>
                    {props.procs.map((p) => (
                        <div className="col-content cpu-percs">
                            {((p.cpu_usage / currentTotal) * 100)
                                .toFixed(1)
                                .padStart(5, " ")}
                            %
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}
