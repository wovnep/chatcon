// import { invoke } from "@tauri-apps/api/tauri";
import ChatArea from "./components/ChatArea";
import InputArea from "./components/InputArea";
import { useEffect } from "react";
import "./App.css"
export default function App() {
	useEffect(() => {
		const textarea = document.getElementById("input-area");
		textarea?.addEventListener("input", () => {
			textarea.style.height='auto';
			textarea.style.height= textarea.scrollHeight+'px';
		})
	}, [])
	return (
		<div className="container">
			<div className="top-container">
				<div className="top-username">
					Ludwig
				</div>
			</div>
			<ChatArea />
			<InputArea />
		</div>
	)
}