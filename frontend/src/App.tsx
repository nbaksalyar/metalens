import React, { useEffect, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import debounce from 'debounce';

import './App.css';
import NodesContainer from './NodesContainer';
// import DataSourceIcon from './data-source.svg';
// import FilterIcon from './filter.svg';

import { addNode, NodeType, Node, nodesCounter } from './nodesSlice';
import { RootState, store } from './store';
import { changeShowGeneratedCode, setAsmCode } from 'uiSlice';
import { setNodeValue } from 'displayPropsSlice';
import ExportImportWindow from 'ExportImportWindow';
import { NavState } from 'interfaces';

const App: React.FC = () => {
  const dispatch = useDispatch();
  const uiState = useSelector((state: RootState) => state.ui);
  const [showExportImport, setShowExportImport] = useState(false);
  const [showDarkBg, setShowDarkBg] = useState(false);

  useEffect(() => {
    // Connect to our WS server
    const ws = new WebSocket("ws://127.0.0.1:8080");

    ws.onmessage = (m) => {
      const { action, payload } = JSON.parse(m.data);

      switch (action) {
        case "asm":
          dispatch(setAsmCode(payload));
          break;

        case "value":
          const value = JSON.parse(payload);
          dispatch(setNodeValue(value));
          break;
      }
    };

    let prevNodesState: Node[] = [];

    // on each state change: debounce 1000 ms and send a new DSL description to the server
    // get back the compiled code
    store.subscribe(debounce(() => {
        const nodesState = store.getState().nodes;
        if (nodesState === prevNodesState) {
          return;
        }
        ws.send(JSON.stringify({ action: 'compile', payload: JSON.stringify(nodesState) }));
        prevNodesState = nodesState;
    }, 500));
  }, [dispatch]);

  const createNode = (type: NodeType) => ((_ev: any) => {
    // FIXME: ugly hack
    nodesCounter.id += 1;

    dispatch(addNode({
      id: nodesCounter.id,
      inputs: null,
      type,
      displayProps: undefined,
    }));
  });

  const showExportImportModal = () => (() => {
    setShowExportImport(!showExportImport);
    showExportImport ? setShowDarkBg(false) : setShowDarkBg(true);
  });

  const closeExportImportModal = () => (() => {
    setShowExportImport(false);
    setShowDarkBg(false);
  });

  const toggleShowCode: React.ChangeEventHandler<HTMLInputElement> = (event) => {
    dispatch(changeShowGeneratedCode(event.target.checked));
  };

  return (
    <div>
      <header className="toolbar">
        <span>
          <span>New Node:</span>
          <button className="toolbarButton" onClick={createNode(NodeType.Filter)}>
            {/* <span className="icon"><img src={FilterIcon} /></span> */}
            <span className="label">Filter</span>
          </button>
          <button className="toolbarButton" onClick={createNode(NodeType.UProbe)}>
            {/* <span className="icon"><img src={DataSourceIcon} /></span> */}
            <span className="label">Data Source</span>
          </button>
          <span>
            <input type="checkbox" id="show-codegen-result" onChange={toggleShowCode} />
            <label htmlFor="show-codegen-result">Show generated code</label>
          </span>
        </span>

        <span>
          <button className="toolbarButton" onClick={showExportImportModal()}>
            {/* <span className="icon"><img src={DataSourceIcon} /></span> */}
            <span className="label">Export/Import Program</span>
          </button>
        </span>
      </header>

      <NodesContainer />

      {uiState.showGeneratedCode && <pre className="codegen-result">{uiState.asmCode}</pre>}

      <div style={{visibility: showDarkBg ? 'visible' : 'hidden'}} className='darkbg'/>


      <ExportImportWindow
        isVisible={showExportImport}
        windowTitle='Export/Import Program'
        state={store.getState()}
        closeExportImportModal={closeExportImportModal} />

    </div>
  );
};

export default App;
