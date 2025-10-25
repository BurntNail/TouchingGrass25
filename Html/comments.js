document.addEventListener("DOMContentLoaded", () => {
    fetch('https://theisland.fly.dev/all_comments').then(rsp => {
        rsp.json().then(json => {
            const chatContainer = document.querySelector(".chat-messages");

            for (let comment of json) {
                const wholeCommentContainer = document.createElement("div");
                const innerDiv = document.createElement("div");
                wholeCommentContainer.classList.add("comment");
                innerDiv.classList.add("comment-content");

                const username = document.createElement("span");
                const text = document.createElement("span");
                username.classList.add("username");
                text.classList.add("text");

                username.innerText = comment.name;
                text.innerText = comment.content;

                innerDiv.appendChild(username);
                innerDiv.appendChild(text);
                wholeCommentContainer.appendChild(innerDiv);
                chatContainer.appendChild(wholeCommentContainer);
            }
        });
    });
});