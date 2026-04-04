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
        <tr>
            <td>Test Level</td>
            <td>100</td>
            <td>dumpling</td>
            <td>50</td>
            <td>5</td>
            <td>50</td>
        </tr>

        {#each data.scores as score}
            <tr>
                <td>{score.level}</td>
                <td>{score.sum}</td>
                <td>{score.username}</td>
                <td>{score.cost}</td>
                <td>{score.cycles}</td>
                <td>{score.area}</td>
            </tr>
        {/each}
    </tbody>
</table>

<style>
    table {
        border: 2px solid black;
    }

    td {
        border: 1px solid gray;
    }
</style>
