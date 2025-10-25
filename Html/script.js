function updateTable (json) {
    let rank = 1;
    const leaderboardTable = document.querySelector("tbody");
    leaderboardTable.innerHTML = "";

    for (let leaderboardEntry of json) {
        let row = document.createElement("tr");

        let rankElement = document.createElement("td");
        rankElement.innerText = rank.toString();
        row.appendChild(rankElement);
        if (rank <= 3) {
            rankElement.style.color = "#06661e";
        } else {
            rankElement.style.color = "black";
        }

        let nameElement = document.createElement("td");
        nameElement.innerText = leaderboardEntry.person;
        row.appendChild(nameElement);
        if (rank <= 3) {
            nameElement.style.color = "#06661e";
        } else {
            nameElement.style.color = "black";
        }

        let scoreElement = document.createElement("td");
        scoreElement.innerText = leaderboardEntry.score;
        row.appendChild(scoreElement);
        if (rank <= 3) {
            scoreElement.style.color = "#06661e";
        } else {
            scoreElement.style.color = "black";
        }

        leaderboardTable.appendChild(row);
        rank++;
    }
}

function updateLeaderboard (json) {
    const imageContainer = document.querySelector(".leaderboard-section");

    for (let old of document.querySelectorAll(".image-column")) {
        old.remove();
    }

    const imageColumn = document.createElement("div");
    imageColumn.classList.add("image-column");

    const nameColumn = document.createElement("div");
    nameColumn.classList.add("image-column");

    for (let imageEntry of json) {
        let frame = document.createElement("div");
        frame.classList.add("image-frame");

        let imageElement = document.createElement("img");
        imageElement.src = imageEntry.image;

        let headingElement = document.createElement("div");
        headingElement.classList.add("name-frame");

        let headingName = document.createElement("h1");
        headingName.innerText = `${imageEntry.person}: \n ${imageEntry.image_score}`;

        headingElement.appendChild(headingName);
        nameColumn.appendChild(headingElement);


        frame.appendChild(imageElement);
        imageColumn.appendChild(frame);

    }

    imageContainer.appendChild(imageColumn);
    imageContainer.appendChild(nameColumn);
}

document.addEventListener("DOMContentLoaded", function () {
    fetch('https://theisland.fly.dev/leaderboard').then(response => {
        response.json().then(updateTable);
    });

    fetch( 'https://theisland.fly.dev/topimages').then(response => {
        response.json().then(updateLeaderboard);
    });
});