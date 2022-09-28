import { EditorLanguage } from "enums";
import { EditorState, EditorView } from "@codemirror/next/basic-setup";
import { lineNumbers } from "@codemirror/next/gutter";
import { highlightActiveLine, highlightSpecialChars, drawSelection, keymap } from "@codemirror/next/view";
import { closeBrackets, closeBracketsKeymap } from '@codemirror/next/closebrackets';
import { defaultKeymap } from '@codemirror/next/commands';
import { highlightSelectionMatches } from '@codemirror/next/search';
import { foldGutter, foldKeymap } from '@codemirror/next/fold';
import { commentKeymap } from '@codemirror/next/comment';
import { lintKeymap } from '@codemirror/next/lint';
import { javascript } from "@codemirror/next/lang-javascript";
import { json } from "@codemirror/next/lang-json";
import React, { useEffect, useRef } from "react";
import { metalensDark } from "./CodeMirrorMetalensTheme";


interface CodeMirrorProps {
    mode: EditorLanguage;
    defaultValue: string;
    // onChange: (event: React.ChangeEvent<HTMLInputElement>) => void;
}

const CodeMirror: React.FC<CodeMirrorProps> = (props) => {

    const editor = useRef<HTMLDivElement>(null);

    let editorMode: any = null;

    switch (props.mode) {
        case EditorLanguage.JS:
            editorMode = javascript;
            break;
        case EditorLanguage.JSON:
            editorMode = json;
            break;
        default:
            editorMode = json;
            break;
    };

    useEffect(() => {
        const cmState = EditorState.create({
            doc: props.defaultValue || 'console.log("Henlo");',
            extensions: [
                lineNumbers(),
                editorMode(),
                highlightActiveLine(),
                highlightSpecialChars(),
                drawSelection(),
                closeBrackets(),
                highlightSelectionMatches(),
                foldGutter(),

                keymap.of([
                    ...closeBracketsKeymap,
                    ...defaultKeymap,
                    ...foldKeymap,
                    ...commentKeymap,
                    ...lintKeymap
                ]),

                metalensDark,
            ],
        });

        if (editor.current) {
            const cmEditor = new EditorView({
                state: cmState,
                parent: editor.current,
            });

            return () => {
                cmEditor.destroy();
            };
        };

    }, []);

    return (<>
        <div
            ref={editor}
            className="codeMirrorDiv"
            spellCheck={false}
        />
    </>);
}

export default CodeMirror;