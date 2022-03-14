import React from "react";
import { Text } from "@blueprintjs/core";
import ReactFlow, { Background, BackgroundVariant } from "react-flow-renderer";

export interface TestingProps {}

const elements = [
    {
        id: "1",
        type: "input", // input node
        data: { label: "Input Node" },
        position: { x: 250, y: 25 },
    },
    // default node
    {
        id: "2",
        // you can also pass a React component as a label
        data: { label: <div>Default Node</div> },
        position: { x: 100, y: 125 },
    },
    {
        id: "3",
        type: "output", // output node
        data: { label: "Output Node" },
        position: { x: 250, y: 250 },
    },
    // animated edge
    { id: "e1-2", source: "1", target: "2", animated: true },
    { id: "e2-3", source: "2", target: "3" },
];

const Testing: React.FC<TestingProps> = (props: TestingProps) => {
    return (
        <div style={{ height: 300 }}>
            <ReactFlow style={{ border: "1px solid black" }} elements={elements}>
                <Background variant={BackgroundVariant.Dots} gap={16} size={1} />
            </ReactFlow>
        </div>
    );
};

export default Testing;
