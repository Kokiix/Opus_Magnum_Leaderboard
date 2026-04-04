<script>
    const { data } = $props();
</script>

<table>
    <thead>
        <tr>
            <th>Level</th>
            <th>Sum</th>
            <th>Winner</th>
            <th>Cost</th>
            <th>Cycles</th>
            <th>Area</th>
        </tr>
    </thead>
    <tbody>
        {#each data.chapterList as chapter}
            <tr><th>{chapter.title}</th></tr>
            {#each chapter.levels as score}
                <tr>
                    <td>{score.level}</td>
                    <td>{score.sum}</td>
                    <td>{score.username}</td>
                    <td>{score.cost}</td>
                    <td>{score.cycles}</td>
                    <td>{score.area}</td>
                </tr>
            {/each}
        {/each}
    </tbody>
</table>

<style>
    table {
        border-collapse: collapse;
        border: 2px solid black;
    }

    td {
        border: 1px solid gray;
    }
</style>
