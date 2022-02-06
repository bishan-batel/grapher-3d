export interface GraphData {
    name: string;
    description: string;
    animate: boolean
    equations: EquationData[];
}

export interface EquationData {
    zIndex: number;
    equation: string;
    disabled: boolean;

    // TODO add amendment Criterion B to remove 'color' attribute
    // color?: string;
}
