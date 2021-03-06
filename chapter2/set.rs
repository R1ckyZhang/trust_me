/**
 * 常用集合类型
 * 1 线性序列
 * 2 kv 提映射表
 * 3 集合类型 有序 无序
 * 4 优先队列
 */

//线性序列 向量
//声明方式为 生命空数组后 push元素进去 或者直接声明多个 统一元素的定长数组 或者使用Vec::New() 进行声明

//线性对接 双端队列 同时具有 先进先出 和后进先出性质的数据结构 元素可以从两端弹出 但是插入和删除操作必须在队列两端进行
//链表 允许在任意一段插入 或者弹出元素 但是通常选择vec VecDeque类型  快速 内存访问效率更高，更好利用cpu缓存
/**
 * 映射表
 * HashMap BTreeMap
 * key 必须为可哈希类型 value为编译阶段 已知大小类型 HashMap 无序 BTreeMap 有序
 */

/**
 * 集合 HashSet BTreeSet
 * 为HashMap BTreeMap 将value设置为空的特定类型 有序性相同
 */

/**
 * 优先队列 基于二叉最大堆实现
 */