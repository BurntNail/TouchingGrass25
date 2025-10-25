document.addEventListener("DOMContentLoaded", function() {
    const url = 'https://theisland.fly.dev/leaderboard';
    const table = document.querySelector("tbody");
    const from_web = fetch(url);
    var rank = 1;
    from_web.then(response => {
        response.json().then(data => {
            for (let leaderboardEntry of data) {
                let row = document.createElement("tr");
               
                let rankElement = document.createElement("td");
                rankElement.innerText = rank;
                row.appendChild(rankElement);
                if(rank <= 3){
                    rankElement.style.color = "LightGreen";
                }else{
                    rankElement.style.color = "black";
                }

                let nameElement = document.createElement("td");
                nameElement.innerText = leaderboardEntry.person;
                row.appendChild(nameElement);
                if(rank <= 3){
                    nameElement.style.color = "LightGreen";
                }else{
                    nameElement.style.color = "black";
                }


                let scoreElement = document.createElement("td");
                scoreElement.innerText = leaderboardEntry.score;
                row.appendChild(scoreElement);
                 if(rank <= 3){
                    scoreElement.style.color = "LightGreen";
                }else{
                    scoreElement.style.color = "black";
                }

                table.appendChild(row);
                rank = rank + 1;
            }
        });
    });
});