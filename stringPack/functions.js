export const appendString = (value) => {
    const text = document.createTextNode(value);
    document.body.appendChild(text);
};