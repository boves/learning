class CommentsController < ApplicationController
  def create
    @article = Article.find(params[:article_id])
    @comment = @article.comments.create(commernt_params)
    redirect_to article_path(@article)
  end

  private 
    def comment_params
      params.require(:comment).permit(:commenter, :boduy)
    end
end
