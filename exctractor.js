// Categories tree builder for YGG
// If you need to update the tree, go te the home page and paste it in the console

function cleanText(text) {
    return text
        .replace(/[\n\t]/g, ' ')
        .replace(/\s+/g, ' ')
        .replace(/ /g, ' ')
        .trim();
}

function parseCategories(rootElement) {
    const categories = [];

    const parentItems = rootElement.querySelectorAll(':scope > li');

    parentItems.forEach(parentLi => {
        const subUl = parentLi.querySelector(':scope > ul');
        if (!subUl) return;

        const parentLink = subUl.querySelector(':scope > li:first-child > a');
        if (!parentLink) return;

        const parentUrl = new URL(parentLink.href);
        const parentParams = new URLSearchParams(parentUrl.search);

        const category = {
            id: parentParams.get('category') || parentLi.className,
            name: cleanText(parentLink.textContent),
            sub_categories: []
        };

        subUl.querySelectorAll(':scope > li:not(:first-child)').forEach(subLi => {
            const subLink = subLi.querySelector('a');
            if (!subLink) return;

            const subUrl = new URL(subLink.href);
            const subParams = new URLSearchParams(subUrl.search);

            category.sub_categories.push({
                id: subParams.get('sub_category') || '',
                name: cleanText(subLink.textContent),
                sub_categories: []
            });
        });

        categories.push(category);
    });

    return categories;
}

const targetElement = document.evaluate(
    '/html/body/div[7]/ul',
    document,
    null,
    XPathResult.FIRST_ORDERED_NODE_TYPE,
    null
).singleNodeValue;

if (targetElement) {
    const categoriesTree = parseCategories(targetElement);
    console.log(JSON.stringify(categoriesTree, null, 2));
} else {
    console.error('Élément non trouvé');
}
