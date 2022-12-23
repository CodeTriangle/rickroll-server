const container = d3.select("#container");

d3.json("/api/stats")
  .then((d) => {
	let keySel = Object.keys(d).sort();

	container.append("ul")
	  .selectAll("li")
	  .data(keySel)
	  .join("li")
	  .text((k) => `${k}: ${d[k]}`)
  })
