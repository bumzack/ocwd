function filterCompanies() {
    const input = document.getElementById('searchBox');
    const filter = input.value.toLowerCase();
    const ul = document.getElementById('companyList');
    const li = ul.getElementsByTagName('li');

    for (let i = 0; i < li.length; i++) {
        const a = li[i].getElementsByTagName('a')[0];
        if (a.innerHTML.toLowerCase().indexOf(filter) > -1) {
            li[i].style.display = '';
        } else {
            li[i].style.display = 'none';
        }
    }
}
