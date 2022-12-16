
export function show_topinteractions_chart(data) {

    let top_3 = data.slice(0, 3);

    let margin = { top: 30, right: 30, bottom: 100, left: 60 },
        width = 900 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    // append svg object
    let svg = d3.select("#chart-topinteractions")
        .append("svg")
        .attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    console.log(top_3);

    let x = d3.scaleBand()
        .range([0, width])
        .domain(top_3.map(function (d) { return d.protocol }))
        .padding(0.6);

    svg.append("g")
        .attr("transform", "translate(0," + height + ")")
        .call(d3.axisBottom(x))
        .selectAll("text")
        .attr("transform", "translate(-10,0)rotate(-45)")
        .style("text-anchor", "end");

    // Add y
    let txs_array = top_3.map(function (d) { return d.txs });
    let y = d3.scaleLinear()
        .domain([0, d3.max(txs_array) + Math.ceil(d3.max(txs_array) * 0.1)])
        .range([height, 0]);

    svg.append("g")
        .call(d3.axisLeft(y).ticks(getTicks(d3.max(txs_array))).tickFormat(d3.format("d")));

    svg.selectAll("mybar")
        .data(top_3)
        .enter()
        .append("rect")
        .attr("x", function (d) { return x(d.protocol); })
        .attr("y", function (d) { return y(0); })
        .attr("width", x.bandwidth())
        .attr("height", function (d) { return height - y(0); })
        .attr("fill", "#fefefe")

    svg.selectAll("rect")
        .transition()
        .duration(800)
        .attr("y", function (d) { return y(d.txs); })
        .attr("height", function (d) { return height - y(d.txs); })
        .delay(function (d, i) { console.log(i); return (i * 100) })
}

// Calculate ticks based on lowest max in array or 10
function getTicks(max) {
    let ticks = 10;
    if (max < 10) {
        ticks = max;
    }
    return ticks;
}



