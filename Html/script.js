document.addEventListener("DOMContentLoaded", function() {
    const url = 'https://theisland.fly.dev/leaderboard';
    const table = document.querySelector("tbody");
    const from_web = fetch(url);
    var rank = 1;
    from_web.then(response => {
        response.json().then(data => {
            for (let leaderboardEntry of data) {
                rank = rank + 1;
                
                let row = document.createElement("tr");


                let rankElement = document.createElement("td");
                rankElement.innerText = rank;
                row.appendChild(rankElement);

                let nameElement = document.createElement("td");
                nameElement.innerText = leaderboardEntry.person;
                row.appendChild(nameElement);

                let scoreElement = document.createElement("td");
                scoreElement.innerText = leaderboardEntry.score;
                row.appendChild(scoreElement);

                table.appendChild(row);
            }
        });
    });
});