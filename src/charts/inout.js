
export function show_inout_chart(data) {

    let margin = { top: 30, right: 30, bottom: 100, left: 60 },
        width = 900 - margin.left - margin.right,
        height = 400 - margin.top - margin.bottom;

    // append svg object
    let svg = d3.select("#chart-inout")
        .append("svg")
        .attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    console.log("Inout data!");
    console.log(data);

    let x = d3.scaleBand()
        .range([0, width])
        .domain(data.map(function (d) { return d.name }))
        .paddingInner(0.3);

    svg.append("g")
        .attr("transform", "translate(0," + height + ")")
        .call(d3.axisBottom(x))
        .selectAll("text")
        .attr("transform", "translate(-10,0)rotate(-45)")
        .style("text-anchor", "end");

    // Add y
    let in_array = data.map(function (d) { return d.inflow });
    let max_inflow = d3.max(in_array) + Math.ceil(d3.max(in_array) * 0.1);

    let out_array = data.map(function (d) { return d.outflow });
    let min_outflow = d3.min(out_array) + Math.ceil(d3.min(out_array) * 0.1);

    let y = d3.scaleLinear()
        .domain([min_outflow, max_inflow])
        .range([height, 0]);

    svg.append("g")
        .call(d3.axisLeft(y).ticks(getTicks(d3.max(in_array))).tickFormat(d3.format("d")));


    // Create In bars
    svg.selectAll("mybar")
        .data(data)
        .enter()
        .append("rect")
        .attr("x", function (d) { return x(d.name); })
        .attr("y", function (d) { return y(d.inflow); })
        .attr("width", x.bandwidth())
        .attr("height", function (d) { return Math.abs(y(0) - y(d.inflow)); })
        .attr("fill", "#42C920")

    svg.selectAll("mybar")
        .data(data)
        .enter()
        .append("rect")
        .attr("x", function (d) { return x(d.name); })
        .attr("y", function (d) { return y(0); })
        .attr("width", x.bandwidth())
        .attr("height", function (d) { return Math.abs(y(d.outflow) - y(0)); })
        .attr("fill", "#E92649")

    svg.append("path")
        .datum(data)
        .attr("fill", "none")
        .attr("stroke", "#fefefe")
        .attr("stroke-width", 2)
        .attr("d", d3.line()
            .x(function (d) { return x(d.name) + x.bandwidth() / 2 })
            .y(function (d) { return y(d.netflow) })
        )

    svg
        .append("g")
        .selectAll("dot")
        .data(data)
        .enter()
        .append("circle")
        .attr("cx", function (d) { return x(d.name) + x.bandwidth() / 2 })
        .attr("cy", function (d) { return y(d.netflow) })
        .attr("r", 5)
        .attr("fill", "#fefefe")



    // svg.selectAll("mybar")
    //     .data(data)
    //     .enter()
    //     .append("rect")
    //     .attr("x", function (d) { return x(d.name); })
    //     .attr("y", function (d) { return y(0); })
    //     .attr("width", x.bandwidth())
    //     .attr("height", function (d) { return height - y(0); })
    //     .attr("fill", "#69b3a2")

    // svg.selectAll("rect")
    //     .transition()
    //     .duration(800)
    //     .attr("y", function (d) { return y(d.txs); })
    //     .attr("height", function (d) { return height - y(d.txs); })
    //     .delay(function (d, i) { console.log(i); return (i * 100) })
}

function getTicks(max) {
    let ticks = 10;
    if (max < 10) {
        ticks = max;
    }
    return ticks;
}


