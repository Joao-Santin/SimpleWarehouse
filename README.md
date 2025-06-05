# **Simple!Warehouse**
-
*intro*

Simple!Warehouse is a Gui written in Rust using a graphical crate called iced.
The intention of this project is to turn the management of a warehouse efficient and simple. 

"Thats all folks!"

---
*necessidades listadas*
    -entrada(numero nf e cfop, quantidades, itens e valores)
    -saida(numero nf, quantidades, itens e valores)
    -estoque atual e historico

---
*structs base*

[x] Company (ainda nao coloquei que e utilizado objectid)
    -id(objectid) - id dentro do mongodb
    -name(String)
    -itype(enum::x) - enum(CompanyTypes)
    -cnpj(String)

[ ] Product
    -id(objectid) - id dentro do mongodb
    -itype(enum::x) - enum(ProductTypes)
    -composition(Vec!) - Vec(products)
    -stock(int) - int
    -ownerid(objectid) - id dentro do mongodb

[ ] Movimentation
    -id(objectid) - id dentro do mongodb
    -date_movimentation - datetime
    -date_made - datetime
    -ownerid - id dentro do mongodb


*enum base*

[x] enum(CompanyTypes)
    -None,
    -Me,
    -Customer,
    -Supplier,

---
*checklist*

[x] iced working(tela teste, text e button com funcao)
[x] mongodb working(post teste)
[] structs and enums described above
[] traits for structs
[] crud
[] interface
[] production
