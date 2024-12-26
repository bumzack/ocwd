document.addEventListener('DOMContentLoaded', function() {
    const urlParams = new URLSearchParams(window.location.search);
    const companyId = urlParams.get('id');

    // Mock data for demonstration purposes
    const companiesData = {
        1: { name: 'Apple Inc.', earningsReports: ['Q1 2023: $50B', 'Q2 2023: $60B'], stockPrices: [/*...*/], dividends: [/*...*/] },
        2: { name: 'Microsoft Corporation', earningsReports: ['Q1 2023: $40B', 'Q2 2023: $50B'], stockPrices: [/*...*/], dividends: [/*...*/] },
        // Add more companies as needed
    };

    const company = companiesData[companyId];
    if (company) {
        document.getElementById('companyName').innerText = company.name;

        const earningsReportsList = document.getElementById('earningsReports');
        earningsReportsList.innerHTML = '';
        company.earningsReports.forEach(report => {
            const li = document.createElement('li');
            li.innerText = report;
            earningsReportsList.appendChild(li);
        });

        // D3.js chart code here
        const margin = { top: 20, right: 30, bottom: 50, left: 40 };
        const width = 800 - margin.left - margin.right;
        const height = 400 - margin.top - margin.bottom;

        const svg = d3.select('#chartContainer').append('svg')
            .attr('width', width + margin.left + margin.right)
            .attr('height', height + margin.top + margin.bottom)
            .append('g')
            .attr('transform', `translate(${margin.left},${margin.top})`);

        // Mock data for stock prices and dividends
        const data = company.stockPrices.map((price, i) => ({ date: new Date(2023, 0, i + 1), price, dividend: company.dividends[i] }));

        const x = d3.scaleTime()
            .domain(d3.extent(data, d => d.date))
            .range([0, width]);

        const yPrice = d3.scaleLinear()
            .domain([0, d3.max(data, d => d.price)])
            .nice()
            .range([height, 0]);

        const yDividend = d3.scaleLinear()
            .domain([0, d3.max(data, d => d.dividend)])
            .nice()
            .range([height, 0]);

        svg.append('g')
            .attr('transform', `translate(0,${height})`)
            .call(d3.axisBottom(x));

        svg.append('g')
            .call(d3.axisLeft(yPrice));

        const priceLine = d3.line()
            .x(d => x(d.date))
            .y(d => yPrice(d.price));

        const dividendLine = d3.line()
            .x(d => x(d.date))
            .y(d => yDividend(d.dividend));

        svg.append('path')
            .datum(data)
            .attr('fill', 'none')
            .attr('stroke', 'steelblue')
            .attr('stroke-width', 1.5)
            .attr('d', priceLine);

        svg.append('path')
            .datum(data)
            .attr('fill', 'none')
            .attr('stroke', 'green')
            .attr('stroke-width', 2)
            .attr('d', dividendLine);

        // Brush and zoom functionality
        const brush = d3.brushX()
            .extent([[0, 0], [width, height]])
            .on('brush end', brushed);

        svg.append('g')
            .attr('class', 'brush')
            .call(brush);

        function brushed({ selection }) {
            if (selection) {
                const [x0, x1] = selection;
                svg.selectAll('.brush').call(brush.move, null); // Clear the brush-based selection
                zoomed([x0, x1].map(d => d3.pointer(d, width)[0]));
            } else {
                zoomed([margin.left, width - margin.right]);
            }
        }

        function zoomed([x0, x1]) {
            const newX = d3.scaleTime()
                .domain(d3.extent(data, d => d.date))
                .range([x0, x1]);

            svg.selectAll('path')
                .datum(data)
                .attr('d', priceLine.x(d => newX(d.date)));
        }
    } else {
        document.getElementById('companyName').innerText = 'Company not found';
    }
});