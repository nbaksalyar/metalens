import React, { createRef, useCallback, useState, } from 'react';
import Xarrow, { Xwrapper, useXarrow } from 'react-xarrows';
import { useSelector } from 'react-redux';
import { NavState } from 'interfaces';

import PanningArea from 'PanningArea';
import { RootState } from './store';
import Node from './Node';
import { setNewState } from 'nodesSlice';

const NodesContainer: React.FC<{}> = () => {
    const nodes = useSelector((state: RootState) => state.nodes);
    const updateArrows = useXarrow();
    const displayProps = useSelector((state: RootState) => state.displayProps);
    const [navState, setNavState] = useState<NavState>({
        initialX: 0,
        initialY: 0,
        offsetX: 0,
        offsetY: 0,
        stepOffsetX: 0,
        stepOffsetY: 0,
        isPanning: false,
        zoom: 1,
      });

    const formatZoom = () => ({transform: `translate(${navState.stepOffsetX}px, ${navState.stepOffsetY}px) scale(${navState.zoom})`});

    const formatTransform = () => (
        { transform: `translate(${navState.offsetX}px, `+
                       `${navState.offsetY}px)` }
    )

    const nodeRefs = nodes.reduce((agg, node) => {
        agg[node['@id']] = createRef();
        return agg;
    }, {} as any);

    const nodeComponents = nodes.map((node) => {
        return (
            <Node key={node['@id']}
                ref={nodeRefs[node['@id']]}
                displayProps={displayProps[node['@id']]}
                node={node} />
        );
    });

    const arrows = nodes.reduce((agg, node) => {
        if (node['@inputs'].length > 0) {
            const myId = node['@id'];
            const myRef = nodeRefs[myId];
            for (const id of node['@inputs']) {
                const arrowComponent = (
                    <Xarrow
                        key={`arrow-${myId}-${id}`}
                        start={`node${id}`}
                        end={myRef}
                        headSize={8}
                        strokeWidth={1.5 * navState.zoom}
                        color={'#DBDBDB'}
                        dashness={true} />
                );
                agg.push(arrowComponent);
            }
        }
        return agg;
    }, [] as any);
/*
    const deselectNode: React.MouseEventHandler = (_event) => {
        console.log('deselect');

        dispatch(changeSelection(null));
    };
*/
    return (
        <>
            <Xwrapper>
                <div className="zoomer" style={formatZoom()}>
                    <div className="nodes" style={formatTransform()}>
                        {nodeComponents}
                    </div>
                </div>
                {arrows}
            </Xwrapper>
            <PanningArea
                navState={navState}
                setNavState={setNavState}
                updateArrows={updateArrows}
            />
        </>
    );
};

export default NodesContainer;
