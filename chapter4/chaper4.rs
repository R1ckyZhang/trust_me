/**
 * 虚拟地址空间
 * 用户空间 内核空间
 * 
 * stack 栈  数据结构 栈内存 入栈 出栈 lifo 后进先出 物理内存不区分堆栈 虚拟内存空间分一部分内存，用于cpu 入栈出栈操作 这部分属于栈内存
 * 栈内存 程序运行过程中保存函数调用所要维护的信息。
 * 栈帧 函数返回地址和参数 ，临时变量（函数内部非静态局部变量和编译器产生的临时变量）  保存的上下文
 rust中 资源管理

 变量   
    敞亮 const 没有内存固定地址 声明周期为全局 随着程序消亡而消亡 编译器关联到使用的地方
 */