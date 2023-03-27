import { getNewMessage } from "../lib/example.chatEvent"
import React, { useState, useEffect } from 'react'
interface Message {
	badge: string[], 
	username: string,
	message: string,
}
export default function Chatarea() {
	const [messages, setMessages]  = useState<Array<Message>>([])
	const [count, setCount] = useState(0);
	useEffect(() => {
		let id = setInterval(() => {
			setMessages(messageArray => [getNewMessage(count), ...messageArray]);
			setCount(count+1)
		  }, 1000);
		return () => clearInterval(id);
	}, [count])
	return (
		<div className="chat-container">
			{messages.map((message, index) => (
			<div key={index} className="message-container">
				<div className="badge-container">
					{message.badge.map((badge, i) => <img key={i} className="badge" src={badge} alt="badge" />)}
				</div>
				<div className="username">
					{message.username}:
				</div>
				<div className="message">
					{message.message}
				</div>
			</div>
		))}
		</div>
	)
}
