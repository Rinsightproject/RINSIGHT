relative_path=$(dirname "$0")

selfpath=$(cd "$(dirname "$0")"; pwd) 

rust_proj_path=$(cd ""${selfpath}"/../.."; pwd)
echo "detected project path : $rust_proj_path"

downloads_path=$(cd ""${rust_proj_path}"/../downloads"; pwd) #可设置
echo "scan path : ${downloads_path}"

arg1="false"
arg2="false"
if [ $# -eq 1 ];
then
    arg1=$1
elif [ $# -eq 2 ];
then
    arg1=$1
    arg2=$2
fi


for cate_dir in $downloads_path/* ;do
    if [ -d $cate_dir ];
    then
        cate_name=`basename $cate_dir`
        echo "scan catgory $cate_name"
        /bin/bash ${relative_path}"/scan-one.sh" ${cate_dir} $arg1 $arg2
    fi
done
