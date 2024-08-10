# InventoryMN

**InventoryMN** is a desktop application, used to manage small home office/workshop inventory.
It allows addition and storing of a different parts, devices, components. Also contains a "Projects" and "Locations" modules.
In the "Projects" module, you can create new projects, preview different statistics, like project completion status, parts used in project and more.
In the "Locations", you can create and store different locations of your included parts e. g. drawers, boxes, bags.

The application is made completely on Rust language, using the [Dioxus Framework](https://dioxuslabs.com/) and [Tailwind CSS](https://tailwindcss.com). It is also relying on **SQLite** as a database, for storing all of the data.

**NOTICE!**: The application is using its own library, as backend and API provider, which is **private**. It was made like that, so you can always provide your own local or cloud based backends and preferred database provider.  


## Screenshots

![Welcome](/public/demo_img/invmn_demo.png?raw=true "Demo welcome page")